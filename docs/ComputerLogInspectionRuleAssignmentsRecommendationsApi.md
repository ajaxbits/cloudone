# \ComputerLogInspectionRuleAssignmentsRecommendationsApi

All URIs are relative to *https://workload.us-1.cloudone.trendmicro.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_log_inspection_rule_ids_to_computer**](ComputerLogInspectionRuleAssignmentsRecommendationsApi.md#add_log_inspection_rule_ids_to_computer) | **POST** /computers/{computerID}/loginspection/assignments | Add Log Inspection Rule IDs
[**list_log_inspection_rule_ids_on_computer**](ComputerLogInspectionRuleAssignmentsRecommendationsApi.md#list_log_inspection_rule_ids_on_computer) | **GET** /computers/{computerID}/loginspection/assignments | List Log Inspection Rule IDs
[**remove_log_inspection_rule_id_from_computer**](ComputerLogInspectionRuleAssignmentsRecommendationsApi.md#remove_log_inspection_rule_id_from_computer) | **DELETE** /computers/{computerID}/loginspection/assignments/{logInspectionRuleID} | Remove a Log Inspection Rule ID
[**set_log_inspection_rule_ids_on_computer**](ComputerLogInspectionRuleAssignmentsRecommendationsApi.md#set_log_inspection_rule_ids_on_computer) | **PUT** /computers/{computerID}/loginspection/assignments | Set Log Inspection Rule IDs



## add_log_inspection_rule_ids_to_computer

> crate::models::LogInspectionAssignments add_log_inspection_rule_ids_to_computer(computer_id, api_version, overrides, rule_ids)
Add Log Inspection Rule IDs

Assign log inspection rule IDs to a computer. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerLogInspectionRuleAssignmentsRecommendationsApi.addLogInspectionRuleIDsToComputer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerLogInspectionRuleAssignmentsRecommendationsApi.add_log_inspection_rule_ids_to_computer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerLogInspectionRuleAssignmentsRecommendationsApi.addLogInspectionRuleIDsToComputer([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**computer_id** | **i32** | The ID number of the computer. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current computer. |  |
**rule_ids** | Option<[**RuleIds**](RuleIds.md)> | The ID numbers of the log inspection rules to add. |  |

### Return type

[**crate::models::LogInspectionAssignments**](logInspectionAssignments.md)

### Authorization

[Legacy_API_Key](../README.md#Legacy_API_Key), [Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_log_inspection_rule_ids_on_computer

> crate::models::LogInspectionAssignments list_log_inspection_rule_ids_on_computer(computer_id, api_version, overrides)
List Log Inspection Rule IDs

Lists all log inspection rule IDs assigned to a computer. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerLogInspectionRuleAssignmentsRecommendationsApi.listLogInspectionRuleIDsOnComputer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerLogInspectionRuleAssignmentsRecommendationsApi.list_log_inspection_rule_ids_on_computer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerLogInspectionRuleAssignmentsRecommendationsApi.listLogInspectionRuleIDsOnComputer([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**computer_id** | **i32** | The ID number of the computer. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current computer. |  |

### Return type

[**crate::models::LogInspectionAssignments**](logInspectionAssignments.md)

### Authorization

[Legacy_API_Key](../README.md#Legacy_API_Key), [Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_log_inspection_rule_id_from_computer

> crate::models::LogInspectionAssignments remove_log_inspection_rule_id_from_computer(computer_id, log_inspection_rule_id, api_version, overrides)
Remove a Log Inspection Rule ID

Unassign a log inspection rule ID from a computer. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerLogInspectionRuleAssignmentsRecommendationsApi.removeLogInspectionRuleIDFromComputer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerLogInspectionRuleAssignmentsRecommendationsApi.remove_log_inspection_rule_id_from_computer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerLogInspectionRuleAssignmentsRecommendationsApi.removeLogInspectionRuleIDFromComputer([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**computer_id** | **i32** | The ID number of the computer. | [required] |
**log_inspection_rule_id** | **i32** | The ID number of the log inspection rule to delete. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current computer. |  |

### Return type

[**crate::models::LogInspectionAssignments**](logInspectionAssignments.md)

### Authorization

[Legacy_API_Key](../README.md#Legacy_API_Key), [Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_log_inspection_rule_ids_on_computer

> crate::models::LogInspectionAssignments set_log_inspection_rule_ids_on_computer(computer_id, api_version, overrides, rule_ids)
Set Log Inspection Rule IDs

Set log inspection rule IDs assigned to a computer. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerLogInspectionRuleAssignmentsRecommendationsApi.setLogInspectionRuleIDsOnComputer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerLogInspectionRuleAssignmentsRecommendationsApi.set_log_inspection_rule_ids_on_computer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerLogInspectionRuleAssignmentsRecommendationsApi.setLogInspectionRuleIDsOnComputer([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**computer_id** | **i32** | The ID number of the computer. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current computer. |  |
**rule_ids** | Option<[**RuleIds**](RuleIds.md)> | The ID numbers of the log inspection rules to set. |  |

### Return type

[**crate::models::LogInspectionAssignments**](logInspectionAssignments.md)

### Authorization

[Legacy_API_Key](../README.md#Legacy_API_Key), [Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

