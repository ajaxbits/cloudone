# \PolicyLogInspectionRuleAssignmentsRecommendationsApi

All URIs are relative to *https://workload.us-1.cloudone.trendmicro.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_log_inspection_rule_ids_to_policy**](PolicyLogInspectionRuleAssignmentsRecommendationsApi.md#add_log_inspection_rule_ids_to_policy) | **POST** /policies/{policyID}/loginspection/assignments | Add Log Inspection Rule IDs
[**list_log_inspection_rule_ids_on_policy**](PolicyLogInspectionRuleAssignmentsRecommendationsApi.md#list_log_inspection_rule_ids_on_policy) | **GET** /policies/{policyID}/loginspection/assignments | List Log Inspection Rule IDs
[**remove_log_inspection_rule_id_from_policy**](PolicyLogInspectionRuleAssignmentsRecommendationsApi.md#remove_log_inspection_rule_id_from_policy) | **DELETE** /policies/{policyID}/loginspection/assignments/{logInspectionRuleID} | Remove a Log Inspection Rule ID
[**set_log_inspection_rule_ids_on_policy**](PolicyLogInspectionRuleAssignmentsRecommendationsApi.md#set_log_inspection_rule_ids_on_policy) | **PUT** /policies/{policyID}/loginspection/assignments | Set Log Inspection Rule IDs



## add_log_inspection_rule_ids_to_policy

> crate::models::LogInspectionAssignments add_log_inspection_rule_ids_to_policy(policy_id, api_version, overrides, rule_ids)
Add Log Inspection Rule IDs

Assign log inspection rule IDs to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleAssignmentsRecommendationsApi.addLogInspectionRuleIDsToPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleAssignmentsRecommendationsApi.add_log_inspection_rule_ids_to_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleAssignmentsRecommendationsApi.addLogInspectionRuleIDsToPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current policy. |  |
**rule_ids** | Option<[**RuleIds**](RuleIds.md)> | The ID numbers of the log inspection rules to add. |  |

### Return type

[**crate::models::LogInspectionAssignments**](logInspectionAssignments.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_log_inspection_rule_ids_on_policy

> crate::models::LogInspectionAssignments list_log_inspection_rule_ids_on_policy(policy_id, api_version, overrides)
List Log Inspection Rule IDs

Lists all log inspection rule IDs assigned to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleAssignmentsRecommendationsApi.listLogInspectionRuleIDsOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleAssignmentsRecommendationsApi.list_log_inspection_rule_ids_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleAssignmentsRecommendationsApi.listLogInspectionRuleIDsOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current policy. |  |

### Return type

[**crate::models::LogInspectionAssignments**](logInspectionAssignments.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_log_inspection_rule_id_from_policy

> crate::models::LogInspectionAssignments remove_log_inspection_rule_id_from_policy(policy_id, log_inspection_rule_id, api_version, overrides)
Remove a Log Inspection Rule ID

Unassign a log inspection rule ID from a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleAssignmentsRecommendationsApi.removeLogInspectionRuleIDFromPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleAssignmentsRecommendationsApi.remove_log_inspection_rule_id_from_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleAssignmentsRecommendationsApi.removeLogInspectionRuleIDFromPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**log_inspection_rule_id** | **i32** | The ID number of the log inspection rule to delete. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current policy. |  |

### Return type

[**crate::models::LogInspectionAssignments**](logInspectionAssignments.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_log_inspection_rule_ids_on_policy

> crate::models::LogInspectionAssignments set_log_inspection_rule_ids_on_policy(policy_id, api_version, overrides, rule_ids)
Set Log Inspection Rule IDs

Set log inspection rule IDs assigned to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleAssignmentsRecommendationsApi.setLogInspectionRuleIDsOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleAssignmentsRecommendationsApi.set_log_inspection_rule_ids_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleAssignmentsRecommendationsApi.setLogInspectionRuleIDsOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current policy. |  |
**rule_ids** | Option<[**RuleIds**](RuleIds.md)> | The ID numbers of the log inspection rules to set. |  |

### Return type

[**crate::models::LogInspectionAssignments**](logInspectionAssignments.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

