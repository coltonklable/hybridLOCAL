<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Price Request</name>
   <tag></tag>
   <elementGuidId>21952540-48a4-4276-a45a-76e7493af6a8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;OfferQueryBuildFromCatalogOfferings\&quot;:{\&quot;fareRuleType\&quot;:\&quot;{{fareRuleType}}\&quot;,\&quot;reCheckInventoryInd\&quot;:\&quot;{{reCheckInventoryInd}}\&quot;,\&quot;lowFareFinderInd\&quot;:\&quot;{{lowFareFinderInd}}\&quot;,\&quot;returnBrandedFaresInd\&quot;:\&quot;{{returnBrandedFaresInd}}\&quot;,\&quot;BuildFromCatalogOfferingsRequest\&quot;:{\&quot;@type\&quot;:\&quot;BuildFromCatalogOfferingsRequestAir\&quot;,\&quot;CatalogOfferingsIdentifier\&quot;:{\&quot;Identifier\&quot;:{\&quot;value\&quot;:\&quot;${ID1}\&quot;}},\&quot;CatalogOfferingIdentifier\&quot;:{\&quot;Identifier\&quot;:{\&quot;value\&quot;:\&quot;${ID2}\&quot;}},\&quot;ProductIdentifier\&quot;:[{\&quot;Identifier\&quot;:{\&quot;value\&quot;:\&quot;${ID3}\&quot;}}]},\&quot;FareRuleCategory\&quot;:[],\&quot;PaymentCriteria\&quot;:{}}}&quot;,
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
      <name>Identity-Key</name>
      <type>Main</type>
      <value>${Identity-Key}</value>
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
   <restUrl>${ts_environment}/${version}/air/price/offers/buildfromcatalogofferings</restUrl>
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
      <id>3c794980-fd11-4c4f-a3f9-0954b19118cc</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>d5a052c0-b7dd-4451-85cd-7f9303e72f81</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>e39f3781-594f-42bd-b7d1-e5a83728c757</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>649c118d-24d0-40ee-aae2-a1f9def9cc76</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Identity</defaultValue>
      <description></description>
      <id>df2edd61-0968-4183-86d2-4c811c85eb2d</id>
      <masked>false</masked>
      <name>Identity-Key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</defaultValue>
      <description></description>
      <id>4d72b96f-f925-467f-acb8-2a9cfc6727c6</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.segment_price_request</defaultValue>
      <description></description>
      <id>90eab828-3c57-488d-bab2-07a4c6318d7c</id>
      <masked>false</masked>
      <name>segment_price_request</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.catalogOfferingsIdentifierValue</defaultValue>
      <description></description>
      <id>bd50b33f-ee4c-4556-be35-41290ae83a6e</id>
      <masked>false</masked>
      <name>ID1</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.catalogOfferingIdentifier</defaultValue>
      <description></description>
      <id>0101655d-cb6f-49e4-851a-156246279975</id>
      <masked>false</masked>
      <name>ID2</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ProductId</defaultValue>
      <description></description>
      <id>96cf2daf-312a-4d6f-ad20-109680414510</id>
      <masked>false</masked>
      <name>ID3</name>
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
