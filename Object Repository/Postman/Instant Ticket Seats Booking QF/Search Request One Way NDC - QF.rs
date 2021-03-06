<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Search Request One Way NDC - QF</name>
   <tag></tag>
   <elementGuidId>964a3e3b-fb4f-4b91-8a7b-fbe27aac55fc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{  \r\n   \&quot;CatalogOfferingsQueryRequest\&quot;:{  \r\n      \&quot;CatalogOfferingsRequest\&quot;:[  \r\n         {  \r\n            \&quot;@type\&quot;:\&quot;CatalogOfferingsRequestAir\&quot;,\r\n            \&quot;maxNumberOfOffersToReturn\&quot;:50,\r\n            \&quot;contentSourceList\&quot;:[\&quot;NDC\&quot;],\r\n            \&quot;offersPerPage\&quot;:100,\r\n            \&quot;returnBrandedFaresInd\&quot;:false,\r\n            \&quot;PassengerCriteria\&quot;:[  \r\n               {  \r\n                  \&quot;number\&quot;:1,\r\n                  \&quot;age\&quot;:25,\r\n                  \&quot;passengerTypeCode\&quot;:\&quot;ADT\&quot;\r\n               }\r\n            ],\r\n            \&quot;SearchCriteriaFlight\&quot;:[  \r\n               {  \r\n                  \&quot;@type\&quot;:\&quot;SearchCriteriaFlight\&quot;,\r\n                  \&quot;departureDate\&quot;:\&quot;${InitialDeparture}\&quot;,\r\n                  \&quot;From\&quot;:{  \r\n                     \&quot;value\&quot;:\&quot;SYD\&quot;\r\n                  },\r\n                  \&quot;To\&quot;:{  \r\n                     \&quot;value\&quot;:\&quot;LHR\&quot;\r\n                  }\r\n               }\r\n            ],\r\n            \&quot;SearchModifiersAir\&quot;:{  \r\n              \&quot;excludeGround\&quot;:\&quot;Train\&quot;,\r\n               \&quot;CarrierPreference\&quot;:[  \r\n                  {  \r\n                     \&quot;type\&quot;:\&quot;Permitted\&quot;,\r\n                     \&quot;carriers\&quot;:[\&quot;QF\&quot;]\r\n                  }\r\n               ]\r\n            }\r\n         }\r\n      ]\r\n   }\r\n}&quot;,
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
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_TERMINAL_OVERRIDE</name>
      <type>Main</type>
      <value>C1FCF5</value>
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
      <id>4da83598-21bf-4511-817e-57e0c9947509</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>c57d730c-28f7-4612-85fc-1d8e9bcdfb4f</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>b5abb476-bdd5-4cf1-bf37-ea7a7edcedc1</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>6efa0076-beed-4ee9-95f5-141619a530cc</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</defaultValue>
      <description></description>
      <id>e773b4bd-777f-49f0-bcdf-3f6569f076bc</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.InitialDeparture</defaultValue>
      <description></description>
      <id>29dbffda-77dc-4932-8ecb-b1ec48aa350b</id>
      <masked>false</masked>
      <name>InitialDeparture</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
