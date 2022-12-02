/**
 * The DefaultController file is a very simple one, which does not need to be changed manually,
 * unless there's a case where business logic routes the request to an entity which is not
 * the service.
 * The heavy lifting of the Controller item is done in Request.js - that is where request
 * parameters are extracted and sent to the service, and where response is handled.
 */

const Controller = require('./Controller');
const service = require('../services/DefaultService');
const getGenerations = async (request, response) => {
  await Controller.handleRequest(request, response, service.getGenerations);
};

const getGenerationsRecordId = async (request, response) => {
  await Controller.handleRequest(request, response, service.getGenerationsRecordId);
};

const getGroups = async (request, response) => {
  await Controller.handleRequest(request, response, service.getGroups);
};

const getGroupsRecordId = async (request, response) => {
  await Controller.handleRequest(request, response, service.getGroupsRecordId);
};

const getUsers = async (request, response) => {
  await Controller.handleRequest(request, response, service.getUsers);
};

const getUsersRecordId = async (request, response) => {
  await Controller.handleRequest(request, response, service.getUsersRecordId);
};

const getUsersSearch = async (request, response) => {
  await Controller.handleRequest(request, response, service.getUsersSearch);
};

const postLogin = async (request, response) => {
  await Controller.handleRequest(request, response, service.postLogin);
};

const putUsers = async (request, response) => {
  await Controller.handleRequest(request, response, service.putUsers);
};


module.exports = {
  getGenerations,
  getGenerationsRecordId,
  getGroups,
  getGroupsRecordId,
  getUsers,
  getUsersRecordId,
  getUsersSearch,
  postLogin,
  putUsers,
};
