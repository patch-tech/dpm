

from typing import Literal

from ..field import StringField, Field, DateTimeField, DateField
from ..field_expr import FieldExpr
from ..table import Table


class ExtProductsMembersChurnMonthly:
    # Source path.
    source_path = "https://api.patch.tech/query/graphql"

    class Map(dict):
        __getattr__ = dict.get

    # Fields.
    fields = Map({
    "pmcm_custom_id": StringField("pmcm_custom_id"),
	"user_id": StringField("user_id"),
	"user_email": StringField("user_email"),
	"users_email_domain": StringField("users_email_domain"),
	"users_phone_number": StringField("users_phone_number"),
	"invoice_id": StringField("invoice_id"),
	"creator_id": StringField("creator_id"),
	"product_id": StringField("product_id"),
	"status_pt_br": StringField("status_pt_br"),
	"payment_method_pt_br": StringField("payment_method_pt_br"),
	"price": Field("price"),
	"paid": Field("paid"),
	"plan_type_pt_br": StringField("plan_type_pt_br"),
	"has_affiliate_normalized": StringField("has_affiliate_normalized"),
	"has_coproducer_normalized": StringField("has_coproducer_normalized"),
	"has_coupon_normalized": StringField("has_coupon_normalized"),
	"paid_at": DateTimeField("paid_at"),
	"refunded_at": DateTimeField("refunded_at"),
	"protested_at": DateTimeField("protested_at"),
	"chargeback_at": DateTimeField("chargeback_at"),
	"canceled_at": DateTimeField("canceled_at"),
	"first_started_product_at": DateTimeField("first_started_product_at"),
	"real_finish_at": DateTimeField("real_finish_at"),
	"last_finish_product_at": DateTimeField("last_finish_product_at"),
	"end_product_subscription_at": DateField("end_product_subscription_at"),
	"real_finish_cohort": DateTimeField("real_finish_cohort"),
	"cohort": DateField("cohort"),
	"churn_detail_pt_br": StringField("churn_detail_pt_br"),
	"user_full_name": StringField("user_full_name"),
	"product_name": StringField("product_name"),
	"ds": DateTimeField("ds")
    })

    # Singleton.
    instance = None
    table_ = None

    def __init__(self):
        self.table_ = Table(
            dataset_name="hubla-prod-bq",
            dataset_version="0.0.1",
            name="ext_products_members_churn_monthly",
            source="https://api.patch.tech/query/graphql",
            fields=list(ExtProductsMembersChurnMonthly.fields.values())
        )
    
    @classmethod
    def get(cls) -> "ExtProductsMembersChurnMonthly":
        if not ExtProductsMembersChurnMonthly.instance:
            ExtProductsMembersChurnMonthly.instance = ExtProductsMembersChurnMonthly()
        return ExtProductsMembersChurnMonthly.instance

    @classmethod
    def table(cls) -> Table:
        return ExtProductsMembersChurnMonthly.get().table_

    @classmethod
    def select(cls, *selection: Literal["pmcm_custom_id"] | Literal["user_id"] | Literal["user_email"] | Literal["users_email_domain"] | Literal["users_phone_number"] | Literal["invoice_id"] | Literal["creator_id"] | Literal["product_id"] | Literal["status_pt_br"] | Literal["payment_method_pt_br"] | Literal["price"] | Literal["paid"] | Literal["plan_type_pt_br"] | Literal["has_affiliate_normalized"] | Literal["has_coproducer_normalized"] | Literal["has_coupon_normalized"] | Literal["paid_at"] | Literal["refunded_at"] | Literal["protested_at"] | Literal["chargeback_at"] | Literal["canceled_at"] | Literal["first_started_product_at"] | Literal["real_finish_at"] | Literal["last_finish_product_at"] | Literal["end_product_subscription_at"] | Literal["real_finish_cohort"] | Literal["cohort"] | Literal["churn_detail_pt_br"] | Literal["user_full_name"] | Literal["product_name"] | Literal["ds"] | FieldExpr) -> Table:
        return ExtProductsMembersChurnMonthly.table().select(*selection)
