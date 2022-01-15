# SearchFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_items** | Option<**i32**> | Limits the number of objects returned. Default 5000. | [optional]
**search_criteria** | Option<[**Vec<crate::models::SearchCriteria>**](searchCriteria.md)> | Array of search critiera used to filter objects. Searching with multiple criteria returns results that satisfy all of the criteria. Searching with no criteria returns all objects. | [optional]
**sort_by_object_id** | Option<**bool**> | If true, forces the response objects to be sorted by ID, overriding the default sort order. Default \"false\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


