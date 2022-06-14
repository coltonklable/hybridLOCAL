<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Search Request One Way NDC - UA_Edreams</name>
   <tag></tag>
   <elementGuidId>80396ac3-9086-4bb4-bcf8-a0b8c529a127</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;CatalogOfferingsQueryRequest\&quot;: {\n        \&quot;CatalogOfferingsRequest\&quot;: [\n            {\n                \&quot;@type\&quot;: \&quot;CatalogOfferingsRequestAir\&quot;,\n                \&quot;maxNumberOfOffersToReturn\&quot;: 200,\n                \&quot;offersPerPage\&quot;: 200,\n                \&quot;contentSourceList\&quot;: [\n                    \&quot;GDS\&quot;\n                ],\n                \&quot;PassengerCriteria\&quot;: [\n                    {\n                        \&quot;number\&quot;: 1,\n                        \&quot;passengerTypeCode\&quot;: \&quot;ADT\&quot;\n                    },\n                    {\n                        \&quot;number\&quot;: 1,\n                        \&quot;passengerTypeCode\&quot;: \&quot;CHD\&quot;\n                    }\n                ],\n                \&quot;SearchCriteriaFlight\&quot;: [\n                    {\n                        \&quot;departureDate\&quot;: \&quot;${DDate}\&quot;,\n                        \&quot;From\&quot;: {\n                            \&quot;value\&quot;: \&quot;${From}\&quot;\n                        },\n                        \&quot;To\&quot;: {\n                            \&quot;value\&quot;: \&quot;${To}\&quot;\n                        }\n                    },\n                    {\n                        \&quot;departureDate\&quot;: \&quot;${RDate}\&quot;,\n                        \&quot;From\&quot;: {\n                            \&quot;value\&quot;: \&quot;${To}\&quot;\n                        },\n                        \&quot;To\&quot;: {\n                            \&quot;value\&quot;: \&quot;${From}\&quot;\n                        }\n                    }\n                ],\n                \&quot;SearchModifiersAir\&quot;: {\n                    \&quot;FlightType\&quot;: {\n                        \&quot;connectionType\&quot;: \&quot;NonStopDirect\&quot;\n                    },\n                    \&quot;CarrierPreference\&quot;: [\n                        {\n                            \&quot;type\&quot;: \&quot;Permitted\&quot;,\n                            \&quot;carriers\&quot;: [\n                                \&quot;KL\&quot;\n                            ]\n                        }\n                    ]\n                },\n                \&quot;PricingModifiersAir\&quot;: {\n                    \&quot;@type\&quot;: \&quot;PricingModifiersAir\&quot;,\n                    \&quot;currencyCode\&quot;: \&quot;AUD\&quot;,\n                    \&quot;FareSelection\&quot;: {\n                        \&quot;@type\&quot;: \&quot;FareSelectionDetail\&quot;,\n                        \&quot;fareType\&quot;: \&quot;PublicFaresOnly\&quot;\n                    }\n                }\n            }\n        ]\n    }\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>${guid}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>${version}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>${version}</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${ts_environment}/uat/${version}/air/search/catalogofferings?view=detail</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ts_environment</defaultValue>
      <description></description>
      <id>f458d17f-7674-4a67-9d7e-8bc8ac1af23c</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>9b43c797-1cc1-46f1-b672-d77213db4347</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>9976b809-06fd-4c9a-aa29-e1ac416c9d1c</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>737ba45d-d1f7-4c27-a734-6331c1b2a369</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</defaultValue>
      <description></description>
      <id>5c313df9-d572-49a0-a7ac-c67cc4f90a65</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</name>
   </variables>
   <variables>
      <defaultValue>'MEL'</defaultValue>
      <description></description>
      <id>05807247-783f-4199-8f25-08797fc26f4f</id>
      <masked>false</masked>
      <name>From</name>
   </variables>
   <variables>
      <defaultValue>'SFO'</defaultValue>
      <description></description>
      <id>dcbb45af-c1aa-4aac-82c4-1b712213c74e</id>
      <masked>false</masked>
      <name>To</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DepDate</defaultValue>
      <description></description>
      <id>37d0b2ad-d129-49c7-a885-9567068bead4</id>
      <masked>false</masked>
      <name>DDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.RetDate</defaultValue>
      <description></description>
      <id>82fbc258-6b39-45a9-966d-9f2916e55eb7</id>
      <masked>false</masked>
      <name>RDate</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
