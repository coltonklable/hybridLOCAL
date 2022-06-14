<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add Offer</name>
   <tag></tag>
   <elementGuidId>15c58174-9cff-4b13-8dd1-d7f2ed4fdcc9</elementGuidId>
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
      <id>559f617b-deac-4b13-9c99-184ecbe8927b</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>84a2c24d-3727-4c4e-982d-36d091387ad5</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.reservationIdUa</defaultValue>
      <description></description>
      <id>eaf2a635-d56c-40c7-92ef-60b97f77aac5</id>
      <masked>false</masked>
      <name>reservationIdUa</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>f1cd15ba-8c7a-4679-b72a-28550948c664</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>ada4f6a1-cd84-4e12-ba48-18d02181553f</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</defaultValue>
      <description></description>
      <id>a78ece38-ba92-48e8-a5be-66032ffbc892</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.segment_price_request</defaultValue>
      <description></description>
      <id>1a2cf549-de16-48db-b238-3a3b610b9677</id>
      <masked>false</masked>
      <name>segment_price_request</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.catalogOfferingsIdentifierValue</defaultValue>
      <description></description>
      <id>b382c221-2d69-4043-850b-2c30b5b32dc6</id>
      <masked>false</masked>
      <name>ID1</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.catalogOfferingIdentifier</defaultValue>
      <description></description>
      <id>a8304b65-7abe-42aa-8555-deb7ce152e6a</id>
      <masked>false</masked>
      <name>ID2</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ProductId</defaultValue>
      <description></description>
      <id>02510225-de2f-40a7-8e7f-70ab99b36c84</id>
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
