<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Search Request One Way NDC - SQ</name>
   <tag></tag>
   <elementGuidId>ee3cb76f-8f86-4164-9c97-21246318662c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{  \r\n   \&quot;CatalogOfferingsQueryRequest\&quot;:{  \r\n      \&quot;CatalogOfferingsRequest\&quot;:[  \r\n         {  \r\n            \&quot;@type\&quot;:\&quot;CatalogOfferingsRequestAir\&quot;,\r\n            \&quot;maxNumberOfOffersToReturn\&quot;:50,\r\n            \&quot;contentSourceList\&quot;:[\&quot;NDC\&quot;],\r\n            \&quot;offersPerPage\&quot;:100,\r\n            \&quot;returnBrandedFaresInd\&quot;:false,\r\n            \&quot;PassengerCriteria\&quot;:[  \r\n               {  \r\n                  \&quot;number\&quot;:1,\r\n                  \&quot;age\&quot;:25,\r\n                  \&quot;passengerTypeCode\&quot;:\&quot;ADT\&quot;\r\n               }\r\n            ],\r\n            \&quot;SearchCriteriaFlight\&quot;:[  \r\n               {  \r\n                  \&quot;@type\&quot;:\&quot;SearchCriteriaFlight\&quot;,\r\n                  \&quot;departureDate\&quot;:\&quot;${InitialDeparture}\&quot;,\r\n                  \&quot;From\&quot;:{  \r\n                     \&quot;value\&quot;:\&quot;SIN\&quot;\r\n                  },\r\n                  \&quot;To\&quot;:{  \r\n                     \&quot;value\&quot;:\&quot;MEL\&quot;\r\n                  }\r\n               }\r\n            ],\r\n            \&quot;SearchModifiersAir\&quot;:{  \r\n            \&quot;excludeGround\&quot;:\&quot;Train\&quot;,\r\n               \&quot;CarrierPreference\&quot;:[  \r\n                  {  \r\n                     \&quot;type\&quot;:\&quot;Preferred\&quot;,\r\n                     \&quot;carriers\&quot;:[\&quot;SQ\&quot;]\r\n                  }\r\n               ]\r\n            }\r\n         }\r\n      ]\r\n   }\r\n}&quot;,
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
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9}</value>
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
      <name>BIN</name>
      <type>Main</type>
      <value>370000</value>
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
      <id>1397b0c7-80c1-4afa-a898-af16519582e5</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>ade9e954-8f60-46e5-817c-db8fb46e1683</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>2257342b-2e67-4deb-a4ac-f5c1d86e26be</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>0c6d9b8d-aa29-46e7-81b3-3f6cbe865303</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9</defaultValue>
      <description></description>
      <id>32ba9fc9-495d-40dd-b28e-3ed2d31136c4</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.InitialDeparture</defaultValue>
      <description></description>
      <id>26b47771-41d2-44f3-9b80-03f241f7ee77</id>
      <masked>false</masked>
      <name>InitialDeparture</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
