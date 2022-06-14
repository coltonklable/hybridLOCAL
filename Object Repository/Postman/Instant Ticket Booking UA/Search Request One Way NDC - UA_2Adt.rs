<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Search Request One Way NDC - UA_2Adt</name>
   <tag></tag>
   <elementGuidId>e878f9ec-d29d-4ffc-9500-f1eed085957c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{  \n   \&quot;CatalogOfferingsQueryRequest\&quot;:{  \n      \&quot;CatalogOfferingsRequest\&quot;:[  \n         {  \n            \&quot;@type\&quot;:\&quot;CatalogOfferingsRequestAir\&quot;,\n            \&quot;maxNumberOfOffersToReturn\&quot;:50,\n            \&quot;contentSourceList\&quot;:[\&quot;NDC\&quot;],\n            \&quot;offersPerPage\&quot;:100,\n            \&quot;returnBrandedFaresInd\&quot;:false,\n            \&quot;PassengerCriteria\&quot;:[  \n               {  \n                  \&quot;number\&quot;:2,\n                  \&quot;age\&quot;:25,\n                  \&quot;passengerTypeCode\&quot;:\&quot;ADT\&quot;\n               }\n            ],\n            \&quot;SearchCriteriaFlight\&quot;:[  \n               {  \n                  \&quot;@type\&quot;:\&quot;SearchCriteriaFlight\&quot;,\n                  \&quot;departureDate\&quot;:\&quot;2021-12-10\&quot;,\n                  \&quot;From\&quot;:{  \n                     \&quot;value\&quot;:\&quot;${From}\&quot;\n                  },\n                  \&quot;To\&quot;:{  \n                     \&quot;value\&quot;:\&quot;${To}\&quot;\n                  }\n               }\n            ],\n            \&quot;SearchModifiersAir\&quot;:{  \n               \&quot;CarrierPreference\&quot;:[  \n                  {  \n                     \&quot;type\&quot;:\&quot;Permitted\&quot;,\n                     \&quot;carriers\&quot;:[\&quot;UA\&quot;]\n                  }\n               ]\n            }\n         }\n      ]\n   }\n}&quot;,
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
   <restUrl>${ts_environment}/${version}/air/search/catalogofferings?view=detail</restUrl>
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
