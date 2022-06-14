<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add Offer</name>
   <tag></tag>
   <elementGuidId>3ac3b824-1794-4617-ab75-d2132d3db960</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;OfferQueryBuildFromCatalogOfferings\&quot;:{\&quot;fareRuleType\&quot;:\&quot;{{fareRuleType}}\&quot;,\&quot;reCheckInventoryInd\&quot;:\&quot;{{reCheckInventoryInd}}\&quot;,\&quot;lowFareFinderInd\&quot;:\&quot;{{lowFareFinderInd}}\&quot;,\&quot;returnBrandedFaresInd\&quot;:\&quot;{{returnBrandedFaresInd}}\&quot;,\&quot;BuildFromCatalogOfferingsRequest\&quot;:{\&quot;@type\&quot;:\&quot;BuildFromCatalogOfferingsRequestAir\&quot;,\&quot;CatalogOfferingsIdentifier\&quot;:{\&quot;Identifier\&quot;:{\&quot;value\&quot;:\&quot;08c7ad87-918d-49fc-bb5e-d2870ed12803\&quot;}},\&quot;CatalogOfferingIdentifier\&quot;:{\&quot;Identifier\&quot;:{\&quot;value\&quot;:\&quot;UA1\&quot;}},\&quot;ProductIdentifier\&quot;:[{\&quot;Identifier\&quot;:{\&quot;value\&quot;:\&quot;UAp0\&quot;}}]},\&quot;FareRuleCategory\&quot;:[],\&quot;PaymentCriteria\&quot;:{}}}&quot;,
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
      <name>trackingId</name>
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
      <name>Content-Version</name>
      <type>Main</type>
      <value>${version}</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${ts_environment}/${version}/air/book/offer/reservationworkbench/${reservationIdUa}/offers/buildfromcatalogofferings</restUrl>
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
      <id>5a09a996-9cc8-46b8-bae9-664134d82518</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>6c2a670b-28bb-4689-96d5-03ffd9fc8d40</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.reservationIdUa</defaultValue>
      <description></description>
      <id>ac7e5601-1a86-410a-bf09-f2207c529839</id>
      <masked>false</masked>
      <name>reservationIdUa</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>164f4ff9-d9f0-4df2-a98b-a2113811c2f1</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>3484f0bb-81f5-4bee-8e93-b42e6ab23c12</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</defaultValue>
      <description></description>
      <id>fffe0c82-70ea-4d6a-8c59-b454faea496c</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.segment_price_request</defaultValue>
      <description></description>
      <id>6148e303-f942-4be0-a367-fd71ad3b6c3e</id>
      <masked>false</masked>
      <name>segment_price_request</name>
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
