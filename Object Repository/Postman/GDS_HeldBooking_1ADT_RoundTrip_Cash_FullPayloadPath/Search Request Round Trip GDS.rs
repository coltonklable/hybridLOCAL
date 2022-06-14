<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Search Request Round Trip GDS</name>
   <tag></tag>
   <elementGuidId>73e492e7-f5e0-48e9-924c-a157e02e908c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;CatalogOfferingsQueryRequest\&quot;: {\n    \&quot;CatalogOfferingsRequest\&quot;: [\n      {\n        \&quot;@type\&quot;: \&quot;CatalogOfferingsRequestAir\&quot;,\n        \&quot;maxNumberOfOffersToReturn\&quot;: 200,\n        \&quot;offersPerPage\&quot;: 200,\n        \&quot;contentSourceList\&quot;: [\n          \&quot;GDS\&quot;\n        ],\n        \&quot;returnBrandedFaresInd\&quot;: true,\n        \&quot;detailViewInd\&quot;: true,\n        \&quot;upsellInd\&quot;: true,\n        \&quot;PassengerCriteria\&quot;: [\n          {\n            \&quot;@type\&quot;: \&quot;PassengerCriteria\&quot;,\n            \&quot;number\&quot;: 1,\n            \&quot;passengerTypeCode\&quot;: \&quot;ADT\&quot;\n          }\n        ],\n        \&quot;SearchCriteriaFlight\&quot;: [\n          {\n            \&quot;departureDate\&quot;: \&quot;2021-12-16\&quot;,\n            \&quot;From\&quot;: {\n              \&quot;value\&quot;: \&quot;SIN\&quot;\n            },\n            \&quot;To\&quot;: {\n              \&quot;value\&quot;: \&quot;BOM\&quot;\n            }\n          },\n          {\n            \&quot;departureDate\&quot;: \&quot;2021-12-21\&quot;,\n            \&quot;From\&quot;: {\n              \&quot;value\&quot;: \&quot;BOM\&quot;\n            },\n            \&quot;To\&quot;: {\n              \&quot;value\&quot;: \&quot;SIN\&quot;\n            }\n          }\n        ],\n        \&quot;SearchModifiersAir\&quot;: {\n          \&quot;excludeGround\&quot;: \&quot;Train\&quot;,\n          \&quot;CabinPreference\&quot;: [\n            {\n              \&quot;preferenceType\&quot;: \&quot;Preferred\&quot;,\n              \&quot;cabins\&quot;: [\n                \&quot;Economy\&quot;\n              ]\n            }\n          ],\n          \&quot;ConnectionPreferences\&quot;: [\n            {\n              \&quot;@type\&quot;: \&quot;ConnectionPreferencesAir\&quot;,\n              \&quot;FlightType\&quot;: {\n                \&quot;connectionType\&quot;: \&quot;DoubleConnection\&quot;\n              }\n            }\n          ]\n        },\n        \&quot;PricingModifiersAir\&quot;: {\n          \&quot;@type\&quot;: \&quot;PricingModifiersAir\&quot;,\n          \&quot;currencyCode\&quot;: \&quot;AUD\&quot;,\n          \&quot;FareSelection\&quot;: {\n            \&quot;@type\&quot;: \&quot;FareSelectionDetail\&quot;,\n            \&quot;fareType\&quot;: \&quot;PublicFaresOnly\&quot;\n          }\n        }\n      }\n    ]\n  }\n}&quot;,
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
      <defaultValue>GlobalVariable.Version</defaultValue>
      <description></description>
      <id>4ad2aad9-4b19-48ae-b840-fa813fd35ae3</id>
      <masked>false</masked>
      <name>Version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>a4fc37b2-82af-4c8b-84f6-8fdcf38e3873</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G</defaultValue>
      <description></description>
      <id>b1b25877-5476-4f95-8857-e17c714a9776</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G</name>
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


GlobalVariable.variable</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
