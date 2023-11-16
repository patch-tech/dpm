package internal

import (
	"context"
	"log"
	"net/url"

	pb "github.com/patch-tech/dpm/internal/backends" // replace with your actual package path

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

type authCreds struct {
	token string
}

func (a *authCreds) GetRequestMetadata(context.Context, ...string) (map[string]string, error) {
	return map[string]string{
		"dpm-auth-token": a.token,
	}, nil
}

func (a *authCreds) RequireTransportSecurity() bool {
	return true
}

func MakeClient(address string, authToken string) pb.DpmAgentClient {
	u, err := url.Parse(address)
	if err != nil {
		log.Fatalf("invalid address: %v", err)
	}

	var opts []grpc.DialOption
	if u.Scheme == "https" {
		creds := credentials.NewClientTLSFromCert(nil, "")
		opts = append(opts, grpc.WithTransportCredentials(creds))
	} else {
		opts = append(opts, grpc.WithInsecure())
	}
	opts = append(opts, grpc.WithPerRPCCredentials(&authCreds{token: authToken}))

	conn, err := grpc.Dial(u.Host, opts...)
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	client := pb.NewDpmAgentClient(conn)
	return client
}
