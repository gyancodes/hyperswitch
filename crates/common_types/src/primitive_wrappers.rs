pub use bool_wrappers::*;

mod bool_wrappers {
    use std::ops::Deref;

    use serde::{Deserialize, Serialize};
    /// Bool that represents if Extended Authorization is Applied or not
    #[derive(
        Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, diesel::expression::AsExpression,
    )]
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub struct ExtendedAuthorizationAppliedBool(bool);
    impl Deref for ExtendedAuthorizationAppliedBool {
        type Target = bool;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl From<bool> for ExtendedAuthorizationAppliedBool {
        fn from(value: bool) -> Self {
            Self(value)
        }
    }
    impl<DB> diesel::serialize::ToSql<diesel::sql_types::Bool, DB> for ExtendedAuthorizationAppliedBool
    where
        DB: diesel::backend::Backend,
        bool: diesel::serialize::ToSql<diesel::sql_types::Bool, DB>,
    {
        fn to_sql<'b>(
            &'b self,
            out: &mut diesel::serialize::Output<'b, '_, DB>,
        ) -> diesel::serialize::Result {
            self.0.to_sql(out)
        }
    }
    impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Bool, DB>
        for ExtendedAuthorizationAppliedBool
    where
        DB: diesel::backend::Backend,
        bool: diesel::deserialize::FromSql<diesel::sql_types::Bool, DB>,
    {
        fn from_sql(value: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
            bool::from_sql(value).map(Self)
        }
    }

    /// Bool that represents if Extended Authorization is Requested or not
    #[derive(
        Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, diesel::expression::AsExpression,
    )]
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub struct RequestExtendedAuthorizationBool(bool);
    impl Deref for RequestExtendedAuthorizationBool {
        type Target = bool;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl From<bool> for RequestExtendedAuthorizationBool {
        fn from(value: bool) -> Self {
            Self(value)
        }
    }
    impl RequestExtendedAuthorizationBool {
        /// returns the inner bool value
        pub fn is_true(&self) -> bool {
            self.0
        }
    }
    impl<DB> diesel::serialize::ToSql<diesel::sql_types::Bool, DB> for RequestExtendedAuthorizationBool
    where
        DB: diesel::backend::Backend,
        bool: diesel::serialize::ToSql<diesel::sql_types::Bool, DB>,
    {
        fn to_sql<'b>(
            &'b self,
            out: &mut diesel::serialize::Output<'b, '_, DB>,
        ) -> diesel::serialize::Result {
            self.0.to_sql(out)
        }
    }
    impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Bool, DB>
        for RequestExtendedAuthorizationBool
    where
        DB: diesel::backend::Backend,
        bool: diesel::deserialize::FromSql<diesel::sql_types::Bool, DB>,
    {
        fn from_sql(value: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
            bool::from_sql(value).map(Self)
        }
    }

    /// Bool that represents if Extended Authorization is always Requested or not
    #[derive(
        Clone, Copy, Debug, Eq, PartialEq, diesel::expression::AsExpression, Serialize, Deserialize,
    )]
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub struct AlwaysRequestExtendedAuthorization(bool);
    impl Deref for AlwaysRequestExtendedAuthorization {
        type Target = bool;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<DB> diesel::serialize::ToSql<diesel::sql_types::Bool, DB>
        for AlwaysRequestExtendedAuthorization
    where
        DB: diesel::backend::Backend,
        bool: diesel::serialize::ToSql<diesel::sql_types::Bool, DB>,
    {
        fn to_sql<'b>(
            &'b self,
            out: &mut diesel::serialize::Output<'b, '_, DB>,
        ) -> diesel::serialize::Result {
            self.0.to_sql(out)
        }
    }
    impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Bool, DB>
        for AlwaysRequestExtendedAuthorization
    where
        DB: diesel::backend::Backend,
        bool: diesel::deserialize::FromSql<diesel::sql_types::Bool, DB>,
    {
        fn from_sql(value: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
            bool::from_sql(value).map(Self)
        }
    }

    /// Bool that represents if Cvv should be collected during payment or not. Default is true
    #[derive(
        Clone, Copy, Debug, Eq, PartialEq, diesel::expression::AsExpression, Serialize, Deserialize,
    )]
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub struct ShouldCollectCvvDuringPayment(bool);
    impl Deref for ShouldCollectCvvDuringPayment {
        type Target = bool;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<DB> diesel::serialize::ToSql<diesel::sql_types::Bool, DB> for ShouldCollectCvvDuringPayment
    where
        DB: diesel::backend::Backend,
        bool: diesel::serialize::ToSql<diesel::sql_types::Bool, DB>,
    {
        fn to_sql<'b>(
            &'b self,
            out: &mut diesel::serialize::Output<'b, '_, DB>,
        ) -> diesel::serialize::Result {
            self.0.to_sql(out)
        }
    }
    impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Bool, DB> for ShouldCollectCvvDuringPayment
    where
        DB: diesel::backend::Backend,
        bool: diesel::deserialize::FromSql<diesel::sql_types::Bool, DB>,
    {
        fn from_sql(value: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
            bool::from_sql(value).map(Self)
        }
    }

    impl Default for ShouldCollectCvvDuringPayment {
        /// Default for `ShouldCollectCvvDuringPayment` is `true`
        fn default() -> Self {
            Self(true)
        }
    }
}
