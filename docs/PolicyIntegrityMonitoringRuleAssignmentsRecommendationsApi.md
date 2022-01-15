# \PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi

All URIs are relative to *https://workload.us-1.cloudone.trendmicro.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_integrity_monitoring_rule_ids_to_policy**](PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.md#add_integrity_monitoring_rule_ids_to_policy) | **POST** /policies/{policyID}/integritymonitoring/assignments | Add Integrity Monitoring Rule IDs
[**list_integrity_monitoring_rule_ids_on_policy**](PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.md#list_integrity_monitoring_rule_ids_on_policy) | **GET** /policies/{policyID}/integritymonitoring/assignments | List Integrity Monitoring Rule IDs
[**remove_integrity_monitoring_rule_id_from_policy**](PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.md#remove_integrity_monitoring_rule_id_from_policy) | **DELETE** /policies/{policyID}/integritymonitoring/assignments/{integrityMonitoringRuleID} | Remove an Integrity Monitoring Rule ID
[**set_integrity_monitoring_rule_ids_on_policy**](PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.md#set_integrity_monitoring_rule_ids_on_policy) | **PUT** /policies/{policyID}/integritymonitoring/assignments | Set Integrity Monitoring Rule IDs



## add_integrity_monitoring_rule_ids_to_policy

> crate::models::IntegrityMonitoringAssignments add_integrity_monitoring_rule_ids_to_policy(policy_id, api_version, overrides, rule_ids)
Add Integrity Monitoring Rule IDs

Assign integrity monitoring rule IDs to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.addIntegrityMonitoringRuleIDsToPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.add_integrity_monitoring_rule_ids_to_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.addIntegrityMonitoringRuleIDsToPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current policy. |  |
**rule_ids** | Option<[**RuleIds**](RuleIds.md)> | The ID numbers of the integrity monitoring rules to add. |  |

### Return type

[**crate::models::IntegrityMonitoringAssignments**](integrityMonitoringAssignments.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_integrity_monitoring_rule_ids_on_policy

> crate::models::IntegrityMonitoringAssignments list_integrity_monitoring_rule_ids_on_policy(policy_id, api_version, overrides)
List Integrity Monitoring Rule IDs

Lists all integrity monitoring rule IDs assigned to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.listIntegrityMonitoringRuleIDsOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.list_integrity_monitoring_rule_ids_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.listIntegrityMonitoringRuleIDsOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current policy. |  |

### Return type

[**crate::models::IntegrityMonitoringAssignments**](integrityMonitoringAssignments.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_integrity_monitoring_rule_id_from_policy

> crate::models::IntegrityMonitoringAssignments remove_integrity_monitoring_rule_id_from_policy(policy_id, integrity_monitoring_rule_id, api_version, overrides)
Remove an Integrity Monitoring Rule ID

Unassign an integrity monitoring rule ID from a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.removeIntegrityMonitoringRuleIDFromPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.remove_integrity_monitoring_rule_id_from_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.removeIntegrityMonitoringRuleIDFromPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**integrity_monitoring_rule_id** | **i32** | The ID number of the integrity monitoring rule to delete. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current policy. |  |

### Return type

[**crate::models::IntegrityMonitoringAssignments**](integrityMonitoringAssignments.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_integrity_monitoring_rule_ids_on_policy

> crate::models::IntegrityMonitoringAssignments set_integrity_monitoring_rule_ids_on_policy(policy_id, api_version, overrides, rule_ids)
Set Integrity Monitoring Rule IDs

Set integrity monitoring rule IDs assigned to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.setIntegrityMonitoringRuleIDsOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.set_integrity_monitoring_rule_ids_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntegrityMonitoringRuleAssignmentsRecommendationsApi.setIntegrityMonitoringRuleIDsOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current policy. |  |
**rule_ids** | Option<[**RuleIds**](RuleIds.md)> | The ID numbers of the integrity monitoring rules to set. |  |

### Return type

[**crate::models::IntegrityMonitoringAssignments**](integrityMonitoringAssignments.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

