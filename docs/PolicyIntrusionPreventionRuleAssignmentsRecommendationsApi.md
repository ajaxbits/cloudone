# \PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi

All URIs are relative to *https://workload.us-1.cloudone.trendmicro.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_intrusion_prevention_rule_ids_to_policy**](PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.md#add_intrusion_prevention_rule_ids_to_policy) | **POST** /policies/{policyID}/intrusionprevention/assignments | Add Intrusion Prevention Rule IDs
[**list_intrusion_prevention_rule_ids_on_policy**](PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.md#list_intrusion_prevention_rule_ids_on_policy) | **GET** /policies/{policyID}/intrusionprevention/assignments | List Intrusion Prevention Rule IDs
[**remove_intrusion_prevention_rule_id_from_policy**](PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.md#remove_intrusion_prevention_rule_id_from_policy) | **DELETE** /policies/{policyID}/intrusionprevention/assignments/{intrusionPreventionRuleID} | Remove an Intrusion Prevention Rule ID
[**set_intrusion_prevention_rule_ids_on_policy**](PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.md#set_intrusion_prevention_rule_ids_on_policy) | **PUT** /policies/{policyID}/intrusionprevention/assignments | Set Intrusion Prevention Rule IDs



## add_intrusion_prevention_rule_ids_to_policy

> crate::models::IntrusionPreventionAssignments add_intrusion_prevention_rule_ids_to_policy(policy_id, api_version, overrides, rule_ids)
Add Intrusion Prevention Rule IDs

Assign intrusion prevention rule IDs to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.addIntrusionPreventionRuleIDsToPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.add_intrusion_prevention_rule_ids_to_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.addIntrusionPreventionRuleIDsToPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current policy. |  |
**rule_ids** | Option<[**RuleIds**](RuleIds.md)> | The ID numbers of the intrusion prevention rules to add. |  |

### Return type

[**crate::models::IntrusionPreventionAssignments**](intrusionPreventionAssignments.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_intrusion_prevention_rule_ids_on_policy

> crate::models::IntrusionPreventionAssignments list_intrusion_prevention_rule_ids_on_policy(policy_id, api_version, overrides)
List Intrusion Prevention Rule IDs

Lists all intrusion prevention rule IDs assigned to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.listIntrusionPreventionRuleIDsOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.list_intrusion_prevention_rule_ids_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.listIntrusionPreventionRuleIDsOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current policy. |  |

### Return type

[**crate::models::IntrusionPreventionAssignments**](intrusionPreventionAssignments.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_intrusion_prevention_rule_id_from_policy

> crate::models::IntrusionPreventionAssignments remove_intrusion_prevention_rule_id_from_policy(policy_id, intrusion_prevention_rule_id, api_version, overrides)
Remove an Intrusion Prevention Rule ID

Unassign an intrusion prevention rule ID from a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.removeIntrusionPreventionRuleIDFromPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.remove_intrusion_prevention_rule_id_from_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.removeIntrusionPreventionRuleIDFromPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**intrusion_prevention_rule_id** | **i32** | The ID number of the intrusion prevention rule to delete. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current policy. |  |

### Return type

[**crate::models::IntrusionPreventionAssignments**](intrusionPreventionAssignments.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_intrusion_prevention_rule_ids_on_policy

> crate::models::IntrusionPreventionAssignments set_intrusion_prevention_rule_ids_on_policy(policy_id, api_version, overrides, rule_ids)
Set Intrusion Prevention Rule IDs

Set intrusion prevention rule IDs assigned to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.setIntrusionPreventionRuleIDsOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.set_intrusion_prevention_rule_ids_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleAssignmentsRecommendationsApi.setIntrusionPreventionRuleIDsOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current policy. |  |
**rule_ids** | Option<[**RuleIds**](RuleIds.md)> | The ID numbers of the intrusion prevention rules to set. |  |

### Return type

[**crate::models::IntrusionPreventionAssignments**](intrusionPreventionAssignments.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

