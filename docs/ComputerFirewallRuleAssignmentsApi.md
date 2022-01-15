# \ComputerFirewallRuleAssignmentsApi

All URIs are relative to *https://workload.us-1.cloudone.trendmicro.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_firewall_rule_ids_to_computer**](ComputerFirewallRuleAssignmentsApi.md#add_firewall_rule_ids_to_computer) | **POST** /computers/{computerID}/firewall/assignments | Add Firewall Rule IDs
[**list_firewall_rule_ids_on_computer**](ComputerFirewallRuleAssignmentsApi.md#list_firewall_rule_ids_on_computer) | **GET** /computers/{computerID}/firewall/assignments | List Firewall Rule IDs
[**remove_firewall_rule_id_from_computer**](ComputerFirewallRuleAssignmentsApi.md#remove_firewall_rule_id_from_computer) | **DELETE** /computers/{computerID}/firewall/assignments/{firewallRuleID} | Remove a Firewall Rule ID
[**set_firewall_rule_ids_on_computer**](ComputerFirewallRuleAssignmentsApi.md#set_firewall_rule_ids_on_computer) | **PUT** /computers/{computerID}/firewall/assignments | Set Firewall Rule IDs



## add_firewall_rule_ids_to_computer

> crate::models::FirewallAssignments add_firewall_rule_ids_to_computer(computer_id, api_version, rule_ids, overrides)
Add Firewall Rule IDs

Assign firewall rule IDs to a computer. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerFirewallRuleAssignmentsApi.addFirewallRuleIDsToComputer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerFirewallRuleAssignmentsApi.add_firewall_rule_ids_to_computer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerFirewallRuleAssignmentsApi.addFirewallRuleIDsToComputer([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**computer_id** | **i32** | The ID number of the computer. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**rule_ids** | [**RuleIds**](RuleIds.md) | The ID numbers of the firewall rules to add. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current computer. |  |

### Return type

[**crate::models::FirewallAssignments**](firewallAssignments.md)

### Authorization

[Legacy_API_Key](../README.md#Legacy_API_Key), [Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_firewall_rule_ids_on_computer

> crate::models::FirewallAssignments list_firewall_rule_ids_on_computer(computer_id, api_version, overrides)
List Firewall Rule IDs

Lists all firewall rule IDs assigned to a computer. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerFirewallRuleAssignmentsApi.listFirewallRuleIDsOnComputer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerFirewallRuleAssignmentsApi.list_firewall_rule_ids_on_computer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerFirewallRuleAssignmentsApi.listFirewallRuleIDsOnComputer([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**computer_id** | **i32** | The ID number of the computer. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current computer. |  |

### Return type

[**crate::models::FirewallAssignments**](firewallAssignments.md)

### Authorization

[Legacy_API_Key](../README.md#Legacy_API_Key), [Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_firewall_rule_id_from_computer

> crate::models::FirewallAssignments remove_firewall_rule_id_from_computer(computer_id, firewall_rule_id, api_version, overrides)
Remove a Firewall Rule ID

Unassign a firewall rule ID from a computer. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerFirewallRuleAssignmentsApi.removeFirewallRuleIDFromComputer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerFirewallRuleAssignmentsApi.remove_firewall_rule_id_from_computer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerFirewallRuleAssignmentsApi.removeFirewallRuleIDFromComputer([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**computer_id** | **i32** | The ID number of the computer. | [required] |
**firewall_rule_id** | **i32** | The ID number of the firewall rule to delete. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current computer. |  |

### Return type

[**crate::models::FirewallAssignments**](firewallAssignments.md)

### Authorization

[Legacy_API_Key](../README.md#Legacy_API_Key), [Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_firewall_rule_ids_on_computer

> crate::models::FirewallAssignments set_firewall_rule_ids_on_computer(computer_id, api_version, overrides, rule_ids)
Set Firewall Rule IDs

Set firewall rule IDs assigned to a computer. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerFirewallRuleAssignmentsApi.setFirewallRuleIDsOnComputer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerFirewallRuleAssignmentsApi.set_firewall_rule_ids_on_computer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerFirewallRuleAssignmentsApi.setFirewallRuleIDsOnComputer([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**computer_id** | **i32** | The ID number of the computer. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Return only rule IDs assigned directly to the current computer. |  |
**rule_ids** | Option<[**RuleIds**](RuleIds.md)> | The ID numbers of the firewall rules to set. |  |

### Return type

[**crate::models::FirewallAssignments**](firewallAssignments.md)

### Authorization

[Legacy_API_Key](../README.md#Legacy_API_Key), [Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

