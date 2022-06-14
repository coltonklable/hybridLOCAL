<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Search Request RoundTrip GDS</name>
   <tag></tag>
   <elementGuidId>f850ebca-2012-444f-88b7-413c1ad06875</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;CatalogOfferingsQueryRequest\&quot;: {\n    \&quot;CatalogOfferingsRequest\&quot;: [\n      {\n        \&quot;@type\&quot;: \&quot;CatalogOfferingsRequestAir\&quot;,\n        \&quot;maxNumberOfOffersToReturn\&quot;: 200,\n        \&quot;offersPerPage\&quot;: 200,\n        \&quot;contentSourceList\&quot;: [\n          \&quot;GDS\&quot;\n        ],\n        \&quot;PassengerCriteria\&quot;: [\n          {\n            \&quot;number\&quot;: 1,\n            \&quot;passengerTypeCode\&quot;: \&quot;ADT\&quot;\n          }\n        ],\n        \&quot;SearchCriteriaFlight\&quot;: [\n          {\n            \&quot;departureDate\&quot;: \&quot;2022-04-10\&quot;,\n            \&quot;From\&quot;: {\n              \&quot;value\&quot;: \&quot;LHR\&quot;\n            },\n            \&quot;To\&quot;: {\n              \&quot;value\&quot;: \&quot;FRA\&quot;\n            }\n          }\n        ],\n        \&quot;SearchModifiersAir\&quot;: {\n          \&quot;FlightType\&quot;: {},\n           \&quot;CarrierPreference\&quot;:[  \n                  {  \n                     \&quot;type\&quot;:\&quot;Permitted\&quot;,\n                     \&quot;carriers\&quot;:[\&quot;LH\&quot;]\n                  }\n               ]\n          \n        },\n        \&quot;PricingModifiersAir\&quot;: {\n          \&quot;@type\&quot;: \&quot;PricingModifiersAir\&quot;,\n          \&quot;currencyCode\&quot;: \&quot;AUD\&quot;,\n          \&quot;FareSelection\&quot;: {\n            \&quot;@type\&quot;: \&quot;FareSelectionDetail\&quot;,\n            \&quot;fareType\&quot;: \&quot;PublicFaresOnly\&quot;\n          }\n        }\n      }\n    ]\n  }\n}&quot;,
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
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>${guid}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>${Version}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>${Version}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api.apim-a.zu2.qa.travelport.io/uat/${Version}/air/search/catalogofferings</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>d4a3a817-7d93-445c-b08d-281c1a61da50</id>
      <masked>false</masked>
      <name>Version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>bef7b9d6-6da2-4757-b824-0c97d285d4a6</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_79JP</defaultValue>
      <description></description>
      <id>7de47eb6-159f-425f-92a0-c6844e2cb18f</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>5d441be3-b53d-46f8-bdbc-d025ed120589</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

println(&quot;Ram Test&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
