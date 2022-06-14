<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Search Request RoundTrip GDS</name>
   <tag></tag>
   <elementGuidId>0f6f7e8a-4ea2-4c59-a449-dd5997317afc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;CatalogOfferingsQueryRequest\&quot;: {\n    \&quot;CatalogOfferingsRequest\&quot;: [\n      {\n        \&quot;@type\&quot;: \&quot;CatalogOfferingsRequestAir\&quot;,\n        \&quot;maxNumberOfOffersToReturn\&quot;: 200,\n        \&quot;offersPerPage\&quot;: 200,\n        \&quot;contentSourceList\&quot;: [\n          \&quot;GDS\&quot;\n        ],\n        \&quot;returnBrandedFaresInd\&quot;: true,\n        \&quot;detailViewInd\&quot;: true,\n        \&quot;upsellInd\&quot;: true,\n        \&quot;PassengerCriteria\&quot;: [\n          {\n            \&quot;@type\&quot;: \&quot;PassengerCriteria\&quot;,\n            \&quot;number\&quot;: 1,\n            \&quot;passengerTypeCode\&quot;: \&quot;ADT\&quot;\n          }\n        ],\n        \&quot;SearchCriteriaFlight\&quot;: [\n          {\n            \&quot;departureDate\&quot;: \&quot;2022-02-16\&quot;,\n            \&quot;From\&quot;: {\n              \&quot;value\&quot;: \&quot;DEN\&quot;\n            },\n            \&quot;To\&quot;: {\n              \&quot;value\&quot;: \&quot;ORD\&quot;\n            }\n          },\n         {\n            \&quot;departureDate\&quot;: \&quot;2022-02-21\&quot;,\n            \&quot;From\&quot;: {\n              \&quot;value\&quot;: \&quot;ORD\&quot;\n            },\n            \&quot;To\&quot;: {\n              \&quot;value\&quot;: \&quot;DEN\&quot;\n            }\n          }\n        ],\n        \&quot;SearchModifiersAir\&quot;: {\n          \&quot;excludeGround\&quot;: \&quot;Train\&quot;,\n          \&quot;CabinPreference\&quot;: [\n            {\n              \&quot;preferenceType\&quot;: \&quot;Preferred\&quot;,\n              \&quot;cabins\&quot;: [\n                \&quot;Economy\&quot;\n              ]\n            }\n          ],\n          \&quot;ConnectionPreferences\&quot;: [\n            {\n              \&quot;@type\&quot;: \&quot;ConnectionPreferencesAir\&quot;,\n              \&quot;FlightType\&quot;: {\n                \&quot;connectionType\&quot;: \&quot;DoubleConnection\&quot;\n              }\n            }\n          ]\n        },\n        \&quot;PricingModifiersAir\&quot;: {\n          \&quot;@type\&quot;: \&quot;PricingModifiersAir\&quot;,\n          \&quot;currencyCode\&quot;: \&quot;AUD\&quot;,\n          \&quot;FareSelection\&quot;: {\n            \&quot;@type\&quot;: \&quot;FareSelectionDetail\&quot;,\n            \&quot;fareType\&quot;: \&quot;PublicFaresOnly\&quot;\n          }\n        }\n      }\n    ]\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>${guid}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>${Version}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
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
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api.apim-a.zu2.qa.travelport.io/uat/${Version}/air/search/catalogofferings</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Version</defaultValue>
      <description></description>
      <id>eea471c0-42a1-4993-b3e2-6f039c854889</id>
      <masked>false</masked>
      <name>Version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>0dbff387-c70b-4993-bcfc-ce8cb02cf0c8</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G</defaultValue>
      <description></description>
      <id>055fa258-b726-4825-b641-f8874e1df7fd</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>092a3653-c3d8-4523-8724-687531f4dc2d</id>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
