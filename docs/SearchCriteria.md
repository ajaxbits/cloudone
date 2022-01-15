# SearchCriteria

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field_name** | Option<**String**> | Name of the field to be tested. Required for all tests except idTest. | [optional]
**boolean_test** | Option<**bool**> | Boolean test, suitable for boolean fields. Default \"true\". | [optional]
**numeric_test** | Option<**String**> | Numeric test, suitable for numeric fields, used in conjuction with the numericValue. Default \"equal\". | [optional]
**numeric_value** | Option<**f64**> | Value used by the numericTest. Required when performing a numericTest. | [optional]
**string_test** | Option<**String**> | String test, suitable for string fields, used in conjuction with the stringValue and stringWildcards. Default \"equal\". | [optional]
**string_value** | Option<**String**> | Value used by the stringTest. Required when performing a stringTest. | [optional]
**string_wildcards** | Option<**bool**> | Controls whether or not wildcard characters (`%` and `_`) are treated as wildcards (true) or regular characters (false). Default \"true\". | [optional]
**choice_test** | Option<**String**> | Choice test, suitable for enum fields, used in conjuction with the choiceValue. Default \"equal\". | [optional]
**choice_value** | Option<**String**> | Value used by the choiceTest. Required when performing a choiceTest. | [optional]
**first_date_value** | Option<**i64**> | First (low) date used to find objects within a date range.  Null (the default) implies no lower limit on the date range. | [optional]
**first_date_inclusive** | Option<**bool**> | Indicates whether the results should include (true) or exclude (false) an exact match for the firstDateValue. Default \"true\". | [optional]
**last_date_value** | Option<**i64**> | Last (high) date used to find objects within a date range.  Null (the default) implies no upper limit on the date range. | [optional]
**last_date_inclusive** | Option<**bool**> | Indicates whether the results should include (true) or exclude (false) an exact match for the lastDateValue. Default \"true\". | [optional]
**null_test** | Option<**bool**> | Null test, suitable for finding fields containing a null value. | [optional]
**version_test** | Option<**String**> | Version test, suitable for version fields, used in conjuction with the versionValue. Default \"equal\". | [optional]
**version_value** | Option<**String**> | Value used by the versionTest. Required when performing a versionTest. | [optional]
**id_value** | Option<**i64**> |  | [optional]
**id_test** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


